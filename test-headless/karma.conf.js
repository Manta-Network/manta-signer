module.exports = function(config) {
    config.set({
        frameworks: ['mocha', 'chai', 'karma-typescript'],
        files: ['test/**/*.ts'],
        reporters: ['progress'],
        port: 9876,
        colors: true,
        preprocessors: {
            "test/**/*.ts": ["karma-typescript"]
        },
        logLevel: config.LOG_INFO,
        browsers: ['ChromeHeadless'],
        autoWatch: false,
        singleRun: true,
        concurrency: Infinity,
    })
}
