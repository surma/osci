module.exports = function(config) {
  const configuration = {
      basePath: '../../',
      frameworks: ['mocha', 'chai', 'karma-typescript'],
      files: [
        {
          pattern: 'tools/osciwasm/src/osciwasm.ts',
          included: false,
        },
        {
          pattern: 'tools/osciwasm/tests/*.test.js',
          type: 'module'
        },
        {
          pattern: 'tools/osciwasm/tests/helpers/**',
          included: false
        },
        {
          pattern: 'target/**',
          included: false
        }
      ],
      mime: {
        'application/wasm': ['wasm']
      },
      preprocessors: {
        '**/*.ts': ['karma-typescript'],
      },
      karmaTypescriptConfig: {
        tsconfig: 'tools/osciwasm/tsconfig.json',
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
      customContextFile: 'tools/osciwasm/tests/context.html',
      customDebugFile: 'tools/osciwasm/tests/debug.html',
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
