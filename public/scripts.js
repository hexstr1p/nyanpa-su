var form = document.getElementById('shorten-form');
var urlBox = form.elements[0];
var link = document.getElementById('link');
var shrBox = document.getElementById('shortened');

// callback to Axios .post().then()
function displayShortenedUrl(res) {
  link.textContext = res.data.shortUrl;
  link.setAttribute('href', res.data.shortUrl);
  shrBox.style.opacity = '1';
  urlBox.value = '';
}

// callback to Axios error handler
function alertError(err) {
  alert('Something with the url is wrong...');
}

form.addEventListener('submit', function(event) {
  event.preventDefault();
  axios.post('/new', { url: urlBox.value })
    .then(displayShotenedUrl)
    .catch(alertError);
});
}
