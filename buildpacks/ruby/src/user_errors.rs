use indoc::formatdoc;
use libherokubuildpack::log as user;

use crate::RubyBuildpackError;

pub(crate) fn on_error(err: libcnb::Error<RubyBuildpackError>) {
    match cause(err) {
        Cause::OurError(error) => log_our_error(error),
        Cause::FrameworkError(error) => user::log_error(
            "heroku/buildpack-ruby internal buildpack error",
            formatdoc! {"
                An unexpected internal error was reported by the framework used
                by this buildpack.

                If the issue persists, consider opening an issue on the GitHub
                repository. If you are unable to deploy to Heroku as a result
                of this issue, consider opening a ticket for additional support.

                Details:

                {error}
            "},
        ),
    };
}

fn log_our_error(error: RubyBuildpackError) {
    match error {
        RubyBuildpackError::RakeDetectError(error) => user::log_error(
            "Error detecting rake tasks",
            format! {"
            The Ruby buildpack uses rake task information from your application to guide
            build logic. Without this information, the Ruby buildpack cannot continue.

            Details:

            {error}

            "},
        ),
        RubyBuildpackError::GemListGetError(error) => user::log_error(
            "Error detecting dependencies",
            format! {"
            The Ruby buildpack uses dependency information from your application to
            guide build logic. Without this information, the Ruby buildpack cannot
            continue.

            Details:

            {error}
            "},
        ),
        RubyBuildpackError::RubyInstallError(error) => user::log_error(
            "Error installing Ruby",
            format! {"
            Could not install the detected Ruby version.

            Details:

            {error}
            "},
        ),
        RubyBuildpackError::MissingGemfileLock(error) => user::log_error(
            "Error: Gemfile.lock required",
            format! {"
            To deploy a Ruby application, a Gemfile.lock file is required in the
            root of your application, but none was found.

            If you have a Gemfile.lock in your application, you may not have it
            tracked in git, or you may be on a different branch.

            Details:

            {error}
            "},
        ),
        RubyBuildpackError::InAppDirCacheError(error) => user::log_error(
            "Internal cache error",
            format! {"
            An internal error occured while caching files.

            Details:

            {error}
            "},
        ),
        RubyBuildpackError::BundleInstallCommandError(error) => user::log_error(
            "Error installing bundler",
            format! {"
            Installation of bundler failed. Bundler is the package managment
            library for Ruby. Bundler is needed to install your application's dependencies
            listed in the Gemfile.

            Command failed:

            {error}
            "},
        ),
        RubyBuildpackError::RakeAssetsPrecompileFailed(error) => user::log_error(
            "Asset compilation failed",
            format! {"
            An error occured while compiling assets via rake command.

            Command failed:

            {error}
            "},
        ),
        RubyBuildpackError::GemInstallBundlerCommandError(error) => user::log_error(
            "Installing gems failed",
            format! {"
            Could not install gems to the system via bundler. Gems are dependencies
            your application listed in the Gemfile and resolved in the Gemfile.lock.

            Command failed:

            {error}
            "},
        ),
    }
}

#[derive(Debug)]
enum Cause {
    OurError(RubyBuildpackError),
    FrameworkError(libcnb::Error<RubyBuildpackError>),
}

fn cause(err: libcnb::Error<RubyBuildpackError>) -> Cause {
    match err {
        libcnb::Error::BuildpackError(err) => Cause::OurError(err),
        err => Cause::FrameworkError(err),
    }
}
