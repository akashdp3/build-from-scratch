import * as net from "net";

import { Connection } from "./http";

// You can use print statements as follows for debugging, they'll be visible when running tests.
console.log("Logs from your program will appear here!");

// Uncomment this to pass the first stage
const server = net.createServer((socket) => {
  console.log("Accepted new connection");

  socket.on("data", (request) => {
    handleConnection(socket, request);
  });

  socket.on("close", () => {
    socket.end();
  });
});

const handleConnection = (socket: any, request: any) => {
  const conn = new Connection(request);
  const [responseHead, responseBody] = conn.generateResponse();

  socket.write(`${responseHead}\r\n`, "utf8");

  if (!socket.write(responseBody)) {
    socket.once("drain", () => {
      socket.write(responseBody);
    });
  }

  socket.end();
};

server.listen(4221, "localhost");
