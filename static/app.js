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
      var wsUri = (window.location.protocol == 'https:' && 'wss://' || 'ws://') + window.location.host + '/agents/stream';
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

      console.log(model.agent_id);

      var row = $("#" + model.agent_id);

      console.log(row);

      var cpu = model["cpu"];
      var cpuClass = "good";
      if (cpu > 50 && cpu < 80) {
        cpuClass = "warn";
      } else if (cpu >= 80) {
        cpuClass = "alert";
      };
      row.find(".cpu").text(cpu + "%");
      row.find(".cpu").removeClass("good warn alert").addClass(cpuClass);

      var memory = model["memory"];
      var memoryClass = "good";
      if (memory > 4000 && memory < 6000) {
        memoryClass = "warn";
      } else if (memory >= 6000) {
        memoryClass = "alert";
      };
      row.find(".memory").text(memory + "MB");
      row.find(".memory").removeClass("good warn alert").addClass(memoryClass);

      row.find(".network_in").text(model["network_in"] + " bytes");
      row.find(".network_out").text(model["network_out"] + " bytes");
    }
  };

  window.clu.connect();
});