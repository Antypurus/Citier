import jetbrains.buildServer.configs.kotlin.*
import jetbrains.buildServer.configs.kotlin.buildFeatures.*
import jetbrains.buildServer.configs.kotlin.triggers.*
import jetbrains.buildServer.configs.kotlin.vcs.GitVcsRoot

/*
    Rust CI/CD chain for TeamCity 2026.1

    Flow:
      Format
        -> BuildDebug   -> TestDebug   \
        -> BuildRelease -> TestRelease  -> AllTestsPassed -> PublishGithubRelease (master only)

    Before use, replace the placeholders below:
      - GIT_REPO_URL     : your git remote URL
      - GITHUB_OWNER     : GitHub org/user, e.g. "Antypurus"
      - GITHUB_REPO      : GitHub repo name
      - BINARY_NAME      : the binary produced by `cargo build`, from Cargo.toml [[bin]] name

    Secure parameters required in the project (add via UI or a separate Kotlin DSL
    "params" block referencing a TeamCity token, never commit the raw value):
      - env.GITHUB_TOKEN : a GitHub PAT with `contents:write` on the target repo
*/

version = "2025.11"

project {
    description = "Rust CI/CD: fmt -> build -> test -> publish (master only)"

    vcsRoot(RustRepo)

    buildType(Format)
    buildType(BuildDebug)
    buildType(BuildRelease)
    buildType(TestDebug)
    buildType(TestRelease)
    buildType(AllTestsPassed)
    buildType(PublishGithubRelease)
}

object RustRepo : GitVcsRoot({
    id("RustRepo")
    name = "Rust Project Repo"
    // Local repo, mounted into the agent container (see docker-compose.yml).
    // This path is as seen INSIDE the agent container, not your host path.
    url = "file:///rust-repo"
    branch = "refs/heads/master"
    branchSpec = "+:refs/heads/*"
})

object Format : BuildType({
    id("Format")
    name = "1. Check Formatting"

    vcs {
        root(RustRepo)
    }

    steps {
        script {
            name = "cargo fmt --check"
            scriptContent = "cargo fmt --all -- --check"
        }
    }

    triggers {
        vcs {
            branchFilter = "+:*"
        }
    }

    features {
        perfmon {}
    }
})

object BuildDebug : BuildType({
    id("BuildDebug")
    name = "2a. Build (Debug)"

    vcs {
        root(RustRepo)
    }

    steps {
        script {
            name = "cargo build"
            scriptContent = "cargo build"
        }
    }

    dependencies {
        snapshot(Format) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
    }

    artifactRules = "target/debug/BINARY_NAME => debug"
})

object BuildRelease : BuildType({
    id("BuildRelease")
    name = "2b. Build (Release)"

    vcs {
        root(RustRepo)
    }

    steps {
        script {
            name = "cargo build --release"
            scriptContent = "cargo build --release"
        }
    }

    dependencies {
        snapshot(Format) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
    }

    artifactRules = "target/release/BINARY_NAME => release"
})

object TestDebug : BuildType({
    id("TestDebug")
    name = "3a. Test (Debug)"

    vcs {
        root(RustRepo)
    }

    steps {
        script {
            name = "cargo test"
            scriptContent = "cargo test"
        }
    }

    dependencies {
        snapshot(BuildDebug) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
    }
})

object TestRelease : BuildType({
    id("TestRelease")
    name = "3b. Test (Release)"

    vcs {
        root(RustRepo)
    }

    steps {
        script {
            name = "cargo test --release"
            scriptContent = "cargo test --release"
        }
    }

    dependencies {
        snapshot(BuildRelease) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
    }
})

// Composite build type: a single green/red status once both test legs pass.
// Also gives PublishGithubRelease one clean thing to hang its finish-build trigger off.
object AllTestsPassed : BuildType({
    id("AllTestsPassed")
    name = "4. All Tests Passed"

    type = BuildTypeSettings.Type.COMPOSITE

    dependencies {
        snapshot(TestDebug) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
        snapshot(TestRelease) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
    }
})

object PublishGithubRelease : BuildType({
    id("PublishGithubRelease")
    name = "5. Publish GitHub Release (master only)"

    vcs {
        root(RustRepo)
    }

    params {
        password("env.GITHUB_TOKEN", "credentialsJSON:REPLACE_WITH_TOKEN_PARAM_ID",
            display = ParameterDisplay.HIDDEN)
        param("env.GITHUB_OWNER", "GITHUB_OWNER")
        param("env.GITHUB_REPO", "GITHUB_REPO")
    }

    dependencies {
        snapshot(AllTestsPassed) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.CANCEL
        }
        // Pull the built release binary out of the build that made it.
        artifacts(BuildRelease) {
            artifactRules = "release/BINARY_NAME => ."
        }
    }

    steps {
        script {
            name = "Create GitHub release and upload binary"
            scriptContent = """
                #!/usr/bin/env bash
                set -euo pipefail

                TAG="v%build.number%"
                REPO="${'$'}GITHUB_OWNER/${'$'}GITHUB_REPO"

                echo "Creating GitHub release ${'$'}TAG for ${'$'}REPO"

                RELEASE_RESPONSE=${'$'}(curl -sf -X POST \
                  -H "Authorization: Bearer ${'$'}GITHUB_TOKEN" \
                  -H "Accept: application/vnd.github+json" \
                  "https://api.github.com/repos/${'$'}REPO/releases" \
                  -d "{\"tag_name\":\"${'$'}TAG\",\"name\":\"${'$'}TAG\",\"generate_release_notes\":true}")

                UPLOAD_URL=${'$'}(echo "${'$'}RELEASE_RESPONSE" | grep -o '"upload_url": *"[^"]*"' | sed -E 's/.*"upload_url": *"([^"{]*)\{.*/\1/')

                echo "Uploading binary asset"
                curl -sf -X POST \
                  -H "Authorization: Bearer ${'$'}GITHUB_TOKEN" \
                  -H "Content-Type: application/octet-stream" \
                  --data-binary @BINARY_NAME \
                  "${'$'}{UPLOAD_URL}?name=BINARY_NAME"

                echo "Release published: ${'$'}TAG"
            """.trimIndent()
        }
    }

    triggers {
        finishBuildTrigger {
            buildType = "${AllTestsPassed.id}"
            successfulOnly = true
            branchFilter = "+:master"
        }
    }
})

