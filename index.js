const express = require('express');
const app = express();
app.use(express.bodyParser());
require('dotenv').config()
PORT = process.env.PORT || 3000;

app.get('/', (req, res) => {
  res.send("Hello!! Where do you want to go?");
});

app.post('/api/dialogflow_line', async (req, res) => {
  const body = req.body();
  console.log('body => ', JSON.stringify(body));
  return res.status(200).send('');
});

app.listen(PORT, () => {
  console.log('Application is running on port ' + PORT);
});