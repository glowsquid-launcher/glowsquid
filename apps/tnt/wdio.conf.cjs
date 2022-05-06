const os = require('os')
const path = require('path')
const { spawn, spawnSync } = require('child_process')

// keep track of the `tauri-driver` child process
let tauriDriver

module.exports.config = {
  specs: ['./src/lib/tests/**/*.js'],
  maxInstances: 1,
  capabilities: [
    {
      maxInstances: 1,
      'tauri:options': {
        application: '../oxidize/target/release/glowsquid'
      }
    }
  ],
  reporters: ['spec'],
  framework: 'mocha',
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000
  },

  // ensure the rust project is built since we expect this binary to exist for the webdriver sessions
  onPrepare: () => spawnSync('yarn', ['workspace', '@glowsquid/oxidize', 'build:cargo']),

  // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests
  beforeSession: () =>
    (tauriDriver = spawn(
      path.resolve(os.homedir(), '.cargo', 'bin', 'tauri-driver'),
      [],
      { stdio: [null, process.stdout, process.stderr] }
    )),

  // clean up the `tauri-driver` process we spawned at the start of the session
  afterSession: () => tauriDriver.kill()
}
