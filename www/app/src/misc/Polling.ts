// wait until the fn return true, timeout and interval are in milliseconds
export function poll(
  fn: () => boolean,
  timeout: number,
  interval: number
): Promise<void> {
  const endTime = Number(new Date()) + timeout;
  const checkCondition = (resolve: any, reject: any) => {
    if (fn()) {
      // If the condition is met, we're done!
      resolve(true);
    } else if (Number(new Date()) < endTime) {
      // If the condition isn't met but the timeout hasn't elapsed, go again
      setTimeout(checkCondition, interval, resolve, reject);
    } else {
      // Didn't match and too much time, reject!
      reject(new Error("timed out for " + fn));
    }
  };

  return new Promise(checkCondition);
}
