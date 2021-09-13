if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('./sw.js').then(reg => {
    console.log('SW registered!');
  }).catch(err => console.log('SW registration FAIL:', err));
}

