module.exports = function(config) {
  const configuration = {
      basePath: '',
      frameworks: ['mocha', 'chai', 'karma-typescript'],
      files: [
        {
          pattern: 'src/osciwasm.ts',
          included: false,
        },
        {
          pattern: 'tests/*.test.js',
          type: 'module'
        }
      ],
      preprocessors: {
        '**/*.ts': ['karma-typescript'],
      },
      karmaTypescriptConfig: {
        tsconfig: './tsconfig.json',
        coverageOptions: {
          exclude: [/\.ts$/i, /.*node_modules.*/]
        },
      },
      reporters: ['progress', 'karma-typescript'],
      port: 9876,
      colors: true,
      logLevel: config.LOG_INFO,
      autoWatch: true,
      singleRun: true,
      concurrency: Infinity,
      // Remove these 2 lines once this PR lands
      // https://github.com/karma-runner/karma/pull/2834
      customContextFile: 'tests/context.html',
      customDebugFile: 'tests/debug.html',
      browsers: ['ChromeCanaryHeadless'],
      customLaunchers: {
        DockerChrome: {
            base: 'ChromeCanaryHeadless',
            flags: ['--no-sandbox'],
        },
      },
    };

    if (process.env.INSIDE_DOCKER)
      configuration.browsers = ['DockerChrome'];

    config.set(configuration);
};
