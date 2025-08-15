import { invoke } from "@tauri-apps/api/core";

// 定義類型顏色對照表
let hashtagColors: { [key: string]: { background: string; text: string } } = {}

// Rust後端函數對照表
const RustFunctions = {
  "readCsvList": "csv_list",
  "readCsvContent": "read_csv",
  "readColors": "read_json_file"
}

let platformSelect: HTMLInputElement | null;
let collectionView: HTMLElement | null;
let loader: HTMLElement | null;
let darkModeSwitch: HTMLInputElement | null;
let searchButton: HTMLButtonElement | null;
let searchBar: HTMLInputElement | null;

/**
  * 從指定的 CSV 檔案中讀取資料，並將每一行資料轉換為 HTML 卡片格式
  * @param {string} platform? - 平台文字 (CSV檔名)
  * @param {Record<string, any>} row - CSV 檔案中的一行資料
  * @returns {Promise<void>} - 返回一個 Promise，表示操作完成
  */
async function displayCsvCard(platform?: string, isReset: Boolean = true) {

  if (!platformSelect || !collectionView) { return; }
  if (searchBar && isReset) { searchBar.value = ""; }

  const filename = (platform || "Linux.csv");
  
  collectionView.innerHTML = "";
  displayLoader();

  try {
    const jsonString = new String(await invoke(RustFunctions["readCsvContent"], { filename: filename })).valueOf();
    const response = JSON.parse(jsonString) as Record<string, any>;
    const array = response.result as Array<Record<string, any>>;
    const keyword = searchBar?.value?.trim();
    const action = highlightAction(keyword);

    for (const row of array) {

      if (keyword) {
        const values = Object.values(row).flat().join(' ');
        if (!values.toLowerCase().includes(keyword.toLowerCase())) continue;
      }

      const divHtml = collectionItemMaker(row, action);
      collectionView.insertAdjacentHTML('beforeend', divHtml);
    }

    await new Promise(resolve => setTimeout(resolve, 1000));

  } catch (error) {
    console.error("Error reading CSV file:", error);
  } finally {
    dismissLoader();
    collectionView.style.opacity = '1.0';
  }
}

/**
  * 將csv檔案的每一行資料轉換為HTML卡片格式
  * @param {Record<string, any>} row - CSV 檔案中的一行資料
  * @param {(text: string) => string} highlight - highlight功能
  * @returns {string} - 返回一個包含 HTML 卡片的字符串
  */
function collectionItemMaker(row: Record<string, any>, highlight: (text: string) => string = (str) => str) {

  const stars = '★'.repeat(Number(row.Level) || 0) + '☆'.repeat(5 - (Number(row.Level) || 0));

  const divHtml = `
    <div class="card">
        <div class="star-rating">${stars}</div>
        <h2 class="card-link"><a href="${row.URL}" target="_blank">${highlight(row.Name)}</a></h2>
        <p class="card-notes">${highlight(row.Notes)}</p>
        ${row.Example ? `<p class="card-example">${highlight(row.Example)}</p>` : ''}
        <div class="hashtag-container">
          ${hashtagHtmlElementMaker(row.Platform, highlight)}
          ${hashtagHtmlElementMaker(row.Type, highlight)}
          ${hashtagHtmlElementMaker(row.OS, highlight)}
          ${hashtagHtmlElementMaker(row.Category, highlight)}
        </div>
    </div>
    `

  return divHtml;
}

/**
  * 將標籤值轉換為 HTML 元素
  * @param {string[]} tags - 標籤值的數組
  * @param {(text: string) => string} highlight - highlight功能
  * @returns {string} - 返回一個包含 HTML 標籤的字符串
  */
function hashtagHtmlElementMaker(tags?: [string], highlight: (text: string) => string = (str) => str) {

  if (!tags) { return '' };
  let hashtags: string[] = [];

  tags.forEach((tag: string) => {
    let color = hashtagColors[tag] || { background: '#e0e0e0', text: '#000000' }; // 預設顏色
    let html = `<span class="hashtag" style="background-color: ${color.background}; color: ${color.text}">${highlight(tag)}</span>`;
    hashtags.push(html);
  });

  return hashtags.join('');
}

/**
  * 將keyword高亮度顯示
  * @param {Record<string, any>} keyword? - 關鍵字
  * @returns {(text: string) => string} - 正規式處理
  */
function highlightAction(keyword?: string) {

    const action = (text: string) => {
      if (!keyword) return text;
      return text.replace(new RegExp(`(${keyword.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`, 'gi'), '<mark>$1</mark>');
    };

    return action;
}

/**
  * 顯示加載器
  * @returns {void}
  */
function displayLoader() { loader?.classList.add('active'); }

/**
  * 隱藏加載器
  * @returns {void}
  */
function dismissLoader() { loader?.classList.remove('active'); }

// [Rust 從零開始網頁爬蟲 | 五倍紅寶石・五倍學院](https://5xcampus.com/posts/hello-world-conf-rust-web-scraping.html)
// 從 Resources 目錄讀取 JSON 檔案 (Tag顏色設定檔)
async function readHashtagColors() {
  try {
    const jsonContent = await invoke(RustFunctions["readColors"], { 
      filename: "hashtagColors.json" 
    }) as string;
    return JSON.parse(jsonContent);
  } catch (error) {
    console.error("無法載入 hashtagColors.json:", error);
    return {};
  }
}

/**
  * 初始化平台選擇器的選項
  * @returns {void}
  */
async function initMenuSetting() {

  if (!platformSelect) return;
  platformSelect.innerHTML = "";  

  let jsonString = new String(await invoke(RustFunctions["readCsvList"])).valueOf();
  let response = JSON.parse(jsonString);
  let list = response.result as string[];

  list.forEach((filename, index) => {

    const option = document.createElement('option');

    if (index === 0) { option.selected = true; }
    option.value = filename;
    option.textContent = filename;

    platformSelect?.appendChild(option);
  });
}

// 初始化深色模式
function initDarkMode() {
  darkModeSwitch = document.querySelector('#switch');
  if (!darkModeSwitch) return;

  // 檢查本地儲存的主題偏好
  const isDarkMode = localStorage.getItem('darkMode') === 'true';
  darkModeSwitch.checked = isDarkMode;
  document.documentElement.classList.toggle('dark-mode', isDarkMode);

  // 監聽切換事件
  darkModeSwitch.addEventListener('change', () => {
    document.documentElement.classList.toggle('dark-mode', darkModeSwitch?.checked);
    localStorage.setItem('darkMode', String(darkModeSwitch?.checked));
  });
}

/**
  * 監聽鍵盤事件，當按下 Command + F 時，將焦點設置到搜索欄
  * @returns {void}
  */
function focusSearchBar() {
  
  document.addEventListener('keydown', (error) => {
    if (error.metaKey && error.key === 'f') {
      error.preventDefault();
      searchBar?.focus();
    }
  });
}

/**
  * 主程式設定，設置平台選擇器和顯示 CSV 卡片
  * @returns {Promise<void>} - 返回一個 Promise，表示操作完成
  */
async function main() {

  platformSelect = document.querySelector("#platform-select");
  collectionView = document.querySelector("#card-collection-view");
  loader = document.querySelector('#loader-container');
  searchButton = document.querySelector("#search-btn");
  searchBar = document.querySelector("#search-bar");

  hashtagColors = await readHashtagColors();
  await initMenuSetting();
  initDarkMode();
  
  if (!platformSelect || !collectionView) { return; }

  if (searchButton && searchBar) {
    searchButton.addEventListener('click', () => displayCsvCard(platformSelect?.value, false));
    searchBar.addEventListener('keydown', (error) => { if (error.key === 'Enter') displayCsvCard(platformSelect?.value, false); });
    focusSearchBar();
  }

  platformSelect?.addEventListener("change", () => displayCsvCard(platformSelect?.value));
  await displayCsvCard(platformSelect.value);
}

main();