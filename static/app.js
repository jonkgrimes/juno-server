$(function() {
  window.clu = {
    conn: null, 

    disconnect: function() {
      if (this.conn != null) {
        console.log("Disconnecting..");
        this.conn.disconnect();
      }
    },

    connect: function () {
      this.disconnect();
      var wsUri = (window.location.protocol == 'https:' && 'wss://' || 'ws://') + window.location.host + '/stream';
      this.conn = new WebSocket(wsUri);
      console.log("Connecting...");
      this.conn.onopen = function() {
        console.log('Connected!');
      };
      this.conn.onclose = function() {
        console.log('Disconnected');
      };
      this.conn.onmessage = function(msg) {
        console.log('Message received: ' + msg.data);
      };
    }
  };

  window.clu.connect();
});