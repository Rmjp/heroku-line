const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.json({ message: 'Ahoy!' });
});

app.listen(443, () => {
  console.log('Application is running on port 9000');
});