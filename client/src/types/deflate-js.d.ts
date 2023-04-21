declare module "deflate-js" {
  function deflate(data: number[], level?: number): number[];
  function inflate(data: number[]): number[];
}
