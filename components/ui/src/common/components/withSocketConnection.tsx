import React from 'react';

function withSocketConnection(wrappedComponent: JSX.Element) {
  return wrappedComponent;
}

export default withSocketConnection;



// let conn = null;

// connect();

// function log(msg) {
//   console.log(msg);
// }

// function connect() {
//   disconnect();
//   const wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
//   conn = new WebSocket(wsUri);
//   log('Connecting...');

//   conn.onopen = function() {
//     log('Connected.');
//   };
//   conn.onmessage = function(e) {
//     log('Received: ' + e.data);
//   };
//   conn.onclose = function() {
//     log('Disconnected.');
//     conn = null;
//   };
// }

// function disconnect() {
//   if (conn != null) {
//     log('Disconnecting...');
//     conn.close();
//     conn = null;
//   }
// }
