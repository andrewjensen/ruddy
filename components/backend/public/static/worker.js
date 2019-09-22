let conn = null;

connect();

function log(msg) {
  console.log(msg);
}

function connect() {
  disconnect();
  const wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
  conn = new WebSocket(wsUri);
  log('Connecting...');

  conn.onopen = function() {
    log('Connected.');
  };
  conn.onmessage = function(e) {
    log('Received: ' + e.data);
    const message = JSON.parse(e.data);
    updateOutput(message);
  };
  conn.onclose = function() {
    log('Disconnected.');
    conn = null;
  };
}

function updateOutput(message) {
  $('#status-output').html(JSON.stringify(message, null, 2));
}

function disconnect() {
  if (conn != null) {
    log('Disconnecting...');
    conn.close();
    conn = null;
  }
}
