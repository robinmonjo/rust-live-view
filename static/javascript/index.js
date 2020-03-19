const socket = new WebSocket('ws://localhost:8080/ws');

socket.addEventListener('open', () => {});

socket.addEventListener('message', (event) => {
  document.getElementById('echo-output').value = event.data;
});


document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('send-btn').onclick = () => {
    const input = document.getElementById('echo-input');
    socket.send(input.value);
    input.value = '';
  };
});
