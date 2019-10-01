import { createTestEnvironment, destroyTestEnvironment } from "./env";

describe("test create & destroy environment", () => {
  it("create & destroy environment", async () => {
    jest.setTimeout(30000);

    const env1 = await createTestEnvironment();
    const env2 = await createTestEnvironment();

    expect(env1).not.toEqual(env2);
    expect(env1.username).not.toBeUndefined();
    expect(env1.password).not.toBeUndefined();

    await expect(destroyTestEnvironment(env1.id)).resolves.toBeTruthy();
    await expect(destroyTestEnvironment(env2.id)).resolves.toBeTruthy();
  });
});
