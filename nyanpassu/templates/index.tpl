<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Nyanpa-su</title>
</head>
<body>
  <h2>Nyanpa-su the URL shortener</h2>
  <h3>{{result}}</h3>
  <form action="/shorten" method="POST">
    <input type="text" name="url" style="width: 35em;"/>
    <input type="submit" value="nyanpassu!"/>
  </form>
  <br>
  <p>{{url_count}} urls shortened</p>
</body>
</html>
