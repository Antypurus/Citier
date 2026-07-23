import jetbrains.buildServer.configs.kotlin.*
import jetbrains.buildServer.configs.kotlin.buildFeatures.*
import jetbrains.buildServer.configs.kotlin.buildSteps.*
import jetbrains.buildServer.configs.kotlin.triggers.*
import jetbrains.buildServer.configs.kotlin.vcs.GitVcsRoot

/*
    SECURITY: every credentialsJSON:... value below is a PLACEHOLDER param ID.
    Create a secure "Password" type parameter in TeamCity's UI (project ->
    Parameters -> Add new parameter), then replace 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
    with that parameter's ID (not the raw token value). Never put an actual
    token string here — the previous token that was pasted into this file is
    considered compromised; revoke it on GitHub and generate a new one.

    Entry point:
      "on_commit" is the single build you click Run on, or that fires on a
      commit/PR. It has no steps — it just snapshot-depends on
      AllTestsPassed, which transitively pulls in the whole upstream chain
      (Format -> Build -> Test) as one build graph. Format/Build/Test/
      AllTestsPassed have NO triggers of their own — only on_commit does —
      so pushing a commit reliably starts the whole chain, not just Format.

    Flow:
      on_commit
        -> AllTestsPassed
             -> TestDebug   -> BuildDebug   -> Format
             -> TestRelease -> BuildRelease -> Format
      PublishGithubRelease runs after AllTestsPassed, master branch only.
*/

version = "2026.1"

project {
    description = "Rust CI/CD: fmt -> build -> test -> publish (master only)"

    vcsRoot(RustRepo)

    buildType(OnCommitRun)

    // Everything below is nested inside a subproject so it collapses under
    // one node in the sidebar. Only "on_commit" shows at the top level;
    // expanding it reveals the individual steps for anyone who wants the
    // per-stage breakdown.
    subProject(OnCommitSteps)
}

object OnCommitSteps : Project({
    id("OnCommitSteps")
    name = "on_commit steps"

    buildType(Format)
    buildType(BuildDebug)
    buildType(BuildRelease)
    buildType(TestDebug)
    buildType(TestRelease)
    buildType(AllTestsPassed)
    buildType(PublishGithubRelease)
})

object RustRepo : GitVcsRoot({
    id("RustRepo")
    name = "Rust Project Repo"
    url = "https://github.com/Antypurus/Citier.git"
    branch = "refs/heads/master"
    // heads/* for normal branches, pull/*/head so TeamCity also discovers PR refs
    branchSpec = "+:refs/heads/*\n+:refs/pull/*/head"
    authMethod = password {
        userName = "Antypurus"
        password = "credentialsJSON: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"
    }
})

// The single entity you click "Run" on, or that fires automatically on a
// commit/PR update. No steps of its own — just snapshot-depends on
// AllTestsPassed, which pulls in the entire upstream chain as one graph.
object OnCommitRun : BuildType({
    id("OnCommitRun")
    name = "on_commit"

    type = BuildTypeSettings.Type.COMPOSITE

    vcs {
        root(RustRepo)
    }

    dependencies {
        snapshot(AllTestsPassed) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
    }

    triggers {
        vcs {
            branchFilter = "+:*"
        }
    }

    features {
        pullRequests {
            vcsRootExtId = "${RustRepo.id}"
            provider = github {
                authType = token {
                    token = "credentialsJSON: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"
                }
                filterAuthorRole = PullRequests.GitHubRoleFilter.MEMBER_OR_COLLABORATOR
            }
        }
    }
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
            scriptContent = """
                set -e
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
                . "${'$'}HOME/.cargo/env"
                cargo fmt --all -- --check
            """.trimIndent()
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
            scriptContent = """
                set -e
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
                . "${'$'}HOME/.cargo/env"
                cargo build
            """.trimIndent()
        }
    }

    dependencies {
        snapshot(Format) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
    }

    artifactRules = "target/debug/Citier => debug"
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
            scriptContent = """
                set -e
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
                . "${'$'}HOME/.cargo/env"
                cargo build --release
            """.trimIndent()
        }
    }

    dependencies {
        snapshot(Format) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
    }

    artifactRules = "target/release/Citier => release"
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
            scriptContent = """
                set -e
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
                . "${'$'}HOME/.cargo/env"
                cargo test
            """.trimIndent()
        }
    }

    dependencies {
        snapshot(BuildDebug) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
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
            scriptContent = """
                set -e
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
                . "${'$'}HOME/.cargo/env"
                cargo test --release
            """.trimIndent()
        }
    }

    dependencies {
        snapshot(BuildRelease) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
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
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
        snapshot(TestRelease) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
    }

    features {
        // The ONLY commit status check posted to GitHub PRs — one clean
        // entry ("All Tests Passed") rather than one per build config.
        commitStatusPublisher {
            vcsRootExtId = "${RustRepo.id}"
            publisher = github {
                githubUrl = "https://api.github.com"
                authType = personalToken {
                    token = "credentialsJSON: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"
                }
            }
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
        password("env.GITHUB_TOKEN", "credentialsJSON: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",
            display = ParameterDisplay.HIDDEN)
        param("env.GITHUB_OWNER", "Antypurus")
        param("env.GITHUB_REPO", "Citier")
    }

    dependencies {
        snapshot(AllTestsPassed) {
            onDependencyFailure = FailureAction.FAIL_TO_START
            onDependencyCancel = FailureAction.FAIL_TO_START
        }
        artifacts(BuildRelease) {
            artifactRules = "release/Citier => ."
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
                  --data-binary @Citier \
                  "${'$'}{UPLOAD_URL}?name=Citier"

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
