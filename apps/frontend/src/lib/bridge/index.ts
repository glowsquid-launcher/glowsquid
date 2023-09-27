import { bindings } from "@glowsquid/tauri-plugin-copper";
import { testConnection as testBindingConnection } from "./bindings";
export * from "./accounts";

/**
 * Tests the connection between the frontend and tauri
 */
export const testConnection = async () => {
  await bindings.testConnection();
  console.log(await testBindingConnection());
};
