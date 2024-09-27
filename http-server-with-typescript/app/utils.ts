export function matchRoute(routePattern: string, url: string) {
  // Convert routePattern into a regular expression by replacing ":param" with "([^/]+)"
  const paramNames: string[] = [];
  const regexPattern = routePattern.replace(/:([^/]+)/g, (match, paramName) => {
    paramNames.push(paramName);
    return "([^/]+)"; // Match any value except "/"
  });

  // Create a regular expression from the route pattern
  const regex = new RegExp(`^${regexPattern}$`);
  const match = url.match(regex);

  if (!match) {
    return null; // No match
  }

  // Extract parameters from the matched URL
  let params = {};
  paramNames.forEach((paramName, index) => {
    params = { ...params, [paramName]: match[index + 1] }; // First capture group starts at index 1
  });

  return params; // Return the extracted parameters
}
