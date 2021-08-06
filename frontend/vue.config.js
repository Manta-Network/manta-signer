module.exports = {
    chainWebpack: config => {
        config.module
            .rule('images')
            .use('url-loader')
            .loader('url-loader')
            .tap(options => Object.assign(options, { limit: 10240 }))
            .end();
        // config.plugins.delete('preload')
        // config.plugins.delete('prefetch')
    }
}