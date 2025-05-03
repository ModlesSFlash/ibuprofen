const { invoke } = window.__TAURI__.core;
const { openUrl } = window.__TAURI__.opener;

let config = null;

document.getElementById("link-tenor-api").addEventListener("click", () => {
  openUrl("https://developers.google.com/tenor/guides/quickstart#setup");
});

document.getElementById("link-google-api").addEventListener("click", () => {
  openUrl("https://developers.google.com/tenor/guides/quickstart#setup");
});

document.getElementById("link-google-cx").addEventListener("click", () => {
  openUrl("https://programmablesearchengine.google.com/controlpanel/all");
});

window.addEventListener("DOMContentLoaded", async () => {
  const is_cfg_missing = await invoke("is_config_missing");
  if (is_cfg_missing) {
    document.getElementById("config-form").style.display = "block";
  } else {
    document.getElementById("app").style.display = "block";
  }
});

async function load_config() {
  config = await invoke("parse_config");
}

document.getElementById("first-launch-form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const data = Object.fromEntries(new FormData(e.target));
  data.resolve_port = parseInt(data.resolve_port);

  try {
    await invoke("save_config", { config: data });
    alert("config saved as \"config.toml\". delete it to see the config setup again (you can also edit the file manually)");
    location.reload();
  } catch (err) {
    alert("failed to save config: " + err);
  }
});

document.getElementById("search-form").addEventListener("submit", async (e) => {
  e.preventDefault();
  document.getElementById("search-btn").click();

  const query = document.getElementById("search-input").value.trim();
  if (!query) return;
  if (!config) await load_config();

  const results_div = document.getElementById("results");
  results_div.innerHTML = "";

  if (document.getElementById("google-check").checked) {
    search_google_images(query, results_div);
  }

  if (document.getElementById("tenor-check").checked) {
    search_tenor(query, results_div);
  }
});

async function search_google_images(query, results_div) {
  const api_key = config.google_api_key;
  const cx = config.google_cx;
  const url = `https://www.googleapis.com/customsearch/v1?key=${api_key}&cx=${cx}&q=${encodeURIComponent(query)}&searchType=image&num=10`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    const items = data.items || [];

    items.forEach((item, index) => {
      const thumb_url = item.image?.thumbnailLink;
      const full_url = item.link;
      const title = item.title || `Image ${index + 1}`;
      const filename = `google_${index + 1}_${title.replace(/[^a-z0-9]/gi, "_").toLowerCase()}`;

      if (thumb_url && full_url) {
        const div = document.createElement("div");
        div.className = "thumb";
        div.innerHTML = `<img src="${thumb_url}" alt="${title}"><div>${index + 1}</div>`;
        div.addEventListener("click", () => {
          invoke("download_png", { url: full_url, filename });
        });
        results_div.appendChild(div);
      }
    });
  } catch (error) {
    console.error("Error fetching Google Images:", error);
  }
}

async function search_tenor(query, results_div) {
  const api_key = config.tenor_api_key;
  const url = `https://tenor.googleapis.com/v2/search?q=${encodeURIComponent(query)}&key=${api_key}&client_key=media_search_plugin&limit=20`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    const results = data.results || [];

    results.forEach((item, index) => {
      const gif_url = item.media_formats?.gif?.url;
      const thumb_url = item.media_formats?.tinygif?.url;
      const filename = `tenor_${index + 1}_${item.content_description.replace(/[^a-z0-9]/gi, "_")}_${item.id}`;

      if (gif_url && thumb_url) {
        const div = document.createElement("div");
        div.className = "thumb";
        div.innerHTML = `<img src="${thumb_url}" alt="GIF ${index + 1}"><div>${index + 1}</div>`;
        div.addEventListener("click", () => {
          invoke("download_gif", { url: gif_url, filename });
        });
        results_div.appendChild(div);
      }
    });
  } catch (error) {
    console.error("Error fetching Tenor GIFs:", error);
  }
}
