export enum Method {
  GET = "GET",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
}

export type MethodType = Method.GET | Method.POST | Method.PUT | Method.DELETE;

export const getMethod = (methodString: string): MethodType => {
  switch (methodString) {
    case "GET":
    default:
      return Method.GET;
    case "POST":
      return Method.POST;
    case "PUT":
      return Method.PUT;
    case "DELETE":
      return Method.DELETE;
  }
};
