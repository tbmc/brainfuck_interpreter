import http from 'http';
import httpProxy from 'http-proxy';

const port = parseInt(process.env.PORT ?? '3000');
const url = `http://127.0.0.1:${port}`;

const proxy = httpProxy.createProxyServer({
    target: url,
    changeOrigin: true,
    secure: false,
});
export const server = http.createServer((req, res) => {
    // console.log(req.url);
    try {
        proxy.web(req, res);
    } catch (err) {
        console.error(err);
    }
});
