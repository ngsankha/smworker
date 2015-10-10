var recv = (function() {
  var callback;

  var _recv = function(callbackFn) {
    if (callbackFn === undefined) {
      return callback;
    } else {
      callback = callbackFn;
    }
  }

  return _recv;
})();
