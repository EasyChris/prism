// Test hot restart functionality
const sqlite3 = require('sqlite3').verbose();
const path = require('path');
const os = require('os');

const dbPath = path.join(os.homedir(), 'Library/Application Support/com.prism.app/logs.db');

console.log('Testing hot restart functionality...\n');

// Function to read config from database
function readConfig() {
  return new Promise((resolve, reject) => {
    const db = new sqlite3.Database(dbPath);
    db.get(
      "SELECT value FROM app_config WHERE key = 'proxy_server_status'",
      (err, row) => {
        db.close();
        if (err) reject(err);
        else resolve(JSON.parse(row.value));
      }
    );
  });
}

// Function to test port availability
function testPort(port) {
  return new Promise((resolve) => {
    const net = require('net');
    const server = net.createServer();

    server.once('error', (err) => {
      if (err.code === 'EADDRINUSE') {
        resolve(true); // Port is in use (server is running)
      } else {
        resolve(false);
      }
    });

    server.once('listening', () => {
      server.close();
      resolve(false); // Port is available (server not running)
    });

    server.listen(port, '127.0.0.1');
  });
}

async function main() {
  try {
    // Check initial status
    console.log('1. Checking initial status...');
    const initialStatus = await readConfig();
    console.log('   Status:', initialStatus);

    const port3000InUse = await testPort(3000);
    console.log('   Port 3000 in use:', port3000InUse);

    if (initialStatus.isRunning && port3000InUse) {
      console.log('   ✅ Server is running on port 3000\n');
    } else {
      console.log('   ❌ Server status mismatch\n');
    }

    console.log('2. Hot restart test instructions:');
    console.log('   - Open the app UI');
    console.log('   - Go to Settings page');
    console.log('   - Change port from 3000 to 8080');
    console.log('   - Click "保存配置" button');
    console.log('   - Wait 2 seconds');
    console.log('   - Run this script again to verify\n');

    console.log('3. Expected results after restart:');
    console.log('   - Port 3000 should be free');
    console.log('   - Port 8080 should be in use');
    console.log('   - Database should show addr: "127.0.0.1:8080"');
    console.log('   - No app restart required!\n');

  } catch (error) {
    console.error('Error:', error);
  }
}

main();
