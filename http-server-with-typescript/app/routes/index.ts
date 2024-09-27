import fs from "fs";

import { Method } from "../http/method";
import { IRequest } from "../http/request";
import { IResponse } from "../http/response";

export const handleEmptyPath = (req: IRequest, res: IResponse) => {
  return res.status(200).toString();
};

export const handleRouteNotFound = (req: IRequest, res: IResponse) => {
  return res.status(404).toString();
};

export const handleGETEchoPath = (
  req: IRequest,
  res: IResponse,
  params: { [key: string]: string },
) => {
  return res
    .status(200)
    .content(params.body)
    .header("Content-Type", "text/plain")
    .header("Content-Length", res.body.length.toString())
    .toString();
};

export const handleGETUserAgentPath = (req: IRequest, res: IResponse) => {
  const userAgent = req.head.headers["User-Agent"];

  return res
    .status(200)
    .content(userAgent)
    .header("Content-Type", "text/plain")
    .header("Content-Length", userAgent.length.toString())
    .toString();
};

export const handleGETFilesPath = (
  req: IRequest,
  res: IResponse,
  params: { [key: string]: string },
) => {
  const args = process.argv.slice(2);
  const [___, absPath] = args;
  const filePath = absPath + "/" + params.fileName;
  try {
    const content = fs.readFileSync(filePath);
    return res
      .status(200)
      .content(content.toString())
      .header("Content-Type", "application/octet-stream")
      .header("Content-Length", content.toString().length.toString())
      .toString();
  } catch (error) {
    return handleRouteNotFound(req, res);
  }
};

export const handlePOSTFilePath = (
  req: IRequest,
  res: IResponse,
  params: { [key: string]: string },
) => {
  const args = process.argv.slice(2);
  const [_, absPath] = args;
  const filePath = absPath + "/" + params.fileName;

  try {
    fs.writeFileSync(filePath, req.body);

    return res.status(201).toString();
  } catch (error) {
    return res.status(500).toString();
  }
};

export default [
  {
    method: Method.GET,
    pattern: "/",
    callback: handleEmptyPath,
  },
  {
    method: Method.GET,
    pattern: "/echo/:body",
    callback: handleGETEchoPath,
  },
  {
    method: Method.GET,
    pattern: "/user-agent",
    callback: handleGETUserAgentPath,
  },
  {
    method: Method.GET,
    pattern: "/files/:fileName",
    callback: handleGETFilesPath,
  },
  {
    method: Method.POST,
    pattern: "/files/:fileName",
    callback: handlePOSTFilePath,
  },
];
