<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Nyanpa-su</title>
  <link rel="stylesheet" href="static/style.css">
</head>
<body>
  <h2>Nyanpa-su the URL shortener</h2>
  <h3>{{result}}</h3>
  <form action="/shorten" method="POST">
    <input id="barr" type="text" name="url"/>
    <input type="submit" value="nyanpassu!"/>
  </form>
  <br>
  <p>{{url_count}} urls shortened</p>
</body>
</html>
