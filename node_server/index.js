import http from 'http';
import { readFile } from 'fs/promises';

const server = http.createServer(async (req, res) => {
  if (req.method === 'GET' && req.url === '/api/orders') {
    try {
      const data = await readFile('../server/data/orders.json', 'utf8');
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(data);
    } catch (err) {
      console.error(err);
      res.writeHead(500, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'Internal Server Error' }));
    }
  } else {
    res.writeHead(404, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ error: 'Not Found' }));
  }
});

server.listen(3000, () => {
  console.log('server running on port 3000');
});
