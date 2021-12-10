self.addEventListener("install", function (event) {
  event.waitUntil(preLoad());
});

const cacheName = "cache-0.0.1";
var preLoad = async function () {
  console.log("[PWA Builder] Install Event processing");
  const cache = await caches.open(cacheName);
  console.log("[PWA Builder] Cached index and offline page during Install");
  return await cache.addAll(["/", "/blog", "/series"]);
};

self.addEventListener("fetch", function (event) {
  if (
    event.request.cache === "only-if-cached" &&
    event.request.mode !== "same-origin"
  ) {
    return;
  }
  console.log("[PWA Builder] The service worker is serving the asset.");
  event.respondWith(
    checkResponse(event.request).catch(function () {
      return returnFromCache(event.request);
    })
  );
  event.waitUntil(addToCache(event.request));
});

var checkResponse = function (request) {
  return new Promise(function (fulfill, reject) {
    fetch(request).then(function (response) {
      if (response.status !== 404) {
        fulfill(response);
      } else {
        reject();
      }
    }, reject);
  });
};

var addToCache = async function (request) {
  const cache = await caches.open(cacheName);
  const response = await fetch(request);
  console.log("[PWA Builder] add page to offline: " + response.url);
  return await cache.put(request, response);
};

var returnFromCache = async function (request) {
  const cache = await caches.open(cacheName);
  const matching = await cache.match(request);
  if (!matching || matching.status == 404) {
    return cache.match("offline.html");
  } else {
    return matching;
  }
};
