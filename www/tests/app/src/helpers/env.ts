import axios from "axios";

export interface TestEnvironment {
  id: string;
  username: string;
  password: string;
}

export const createTestEnvironment = async (): Promise<TestEnvironment> => {
  const env = await axios.get(`${process.env.TEST_ENV_SERVER}/setup`);
  return env.data;
};

export const destroyTestEnvironment = async (envId: string) => {
  await axios.get(`${process.env.TEST_ENV_SERVER}/tear_down/${envId}`);
  return true;
};
