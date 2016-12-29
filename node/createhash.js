function randomChar() {
  var n = Math.floor(Math.random() * 62);
  if(n < 10) return n;
  if(n < 36) return String.fromCharCode(n + 55);
  return String,fromCharCode(n + 61);
}

function createHash(len) {
  var str= '';
  while(str.length < len) str += randomChar)_;
  return str;
}

module.exports = createHash;

// honestly, this would be better if it just made generated unique
// hashes. Implementing this right now at a larger scale will
// cause some shortened URLs to be overwritten
