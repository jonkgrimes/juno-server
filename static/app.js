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
        window.clu.writeMessage(msg);
      };
    },

    writeMessage: function(message) {
      var model = JSON.parse(message.data);

      var cpu = model["cpu"];
      var cpuClass = "good";
      if (cpu > 50 && cpu < 80) {
        cpuClass = "warn";
      } else if (cpu >= 80) {
        cpuClass = "alert";
      };
      $("#cpu").text(cpu + "%");
      $("#cpu").removeClass("good warn alert").addClass(cpuClass);

      var memory = model["memory"];
      var memoryClass = "good";
      if (memory > 4000 && memory < 6000) {
        memoryClass = "warn";
      } else if (memory >= 6000) {
        memoryClass = "alert";
      };
      $("#memory").text(memory + "MB");
      $("#memory").removeClass("good warn alert").addClass(memoryClass);

      $("#network_in").text(model["network_in"] + " bytes");
      $("#network_out").text(model["network_out"] + " bytes");
    }
  };

  window.clu.connect();
});