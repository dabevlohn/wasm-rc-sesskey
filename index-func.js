var xheader = "-00-";
function headRequest(url, callback) {
  let headReq = new XMLHttpRequest();
  headReq.onreadystatechange = function () {
    if (this.readyState === 4) {
      if (callback && typeof callback === "function") {
        // TODO: turn header name to X-Email!
        callback(this.getResponseHeader("content-type"));
      }
    }
  };
  headReq.open("HEAD", url, true);
  headReq.send(null);
}
function getCurrentOrigin() {
  var origin = window.location.origin;
  if (origin == "null") {
    origin = window.location.protocol + "//" + window.location.host;
  }
  return origin;
}
function getCurrentPathname() {
  var pathname = window.location.pathname;
  if (pathname == "/") {
    pathname += "index.html";
  }
  return pathname;
}
function getXemailHeader() {
  return xheader;
}
headRequest(window.location.href, (header) => {
  xheader = header;
});
