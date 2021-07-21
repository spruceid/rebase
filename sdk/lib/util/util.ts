// Is there a better way?
export default function desync<T>(f: (...n: any[]) => T): (...n: any[]) => Promise<T> {
  return (...n) => new Promise((resolve) => resolve(f(...n)));
}
