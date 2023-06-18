use web_sys::{ window, Window };

pub fn get_window_size() -> Option<(u32, u32)> {
    let window: Window = window().unwrap();
    if let Some(document) = window.document() {
        if let Some(document_element) = document.document_element() {
            let width = document_element.client_width();
            let height = document_element.client_height();
            return Some((width as u32, height as u32));
        }
    }

    None
}

// struct GithubStats {
    // url: String,
    // repo_url: String
// }

pub fn get_github_data() -> Vec<f64> {
    return vec![];
}

// class Github {
  // constructor() {
    // this.url = "https://api.github.com/users/";
    // this.repoUrl = "https://api.github.com/repos/";
  // }
  // async getGithubData(username) {
    // const responseUser = await fetch(this.url + username);
    // const responseRepo = await fetch(this.url + username + "/repos");

    // const userData = await responseUser.json();
    // const repoData = await responseRepo.json();

    // set language percentage of each repo
    // for (let i in repoData) {
      // get language percentage of repo
      // repoData[i].languagesPercentage = await this.getRepoLanguagePercentage(
        // username,
        // repoData[i].name
      // );
    // }

    // return {
      // user: userData,
      // repo: repoData,
    // };
  // }

  // async getRepoLanguagePercentage(username, reponame) {
    // const ls = await fetch(
      // this.repoUrl + username + "/" + reponame + "/languages"
    // );

    // const languageStats = await ls.json();

    // const totalPtsArr = Object.values(languageStats);
    // var sumTotalPts = 0;
    // totalPtsArr.forEach((pts) => {
      // sumTotalPts += pts;
    // });

    // const languagesPercentage = {};
    // Object.keys(languageStats).forEach((lang) => {
      // languagesPercentage[lang] = (languageStats[lang] * 100) / sumTotalPts;
    // });

    // return languagesPercentage;
  // }
// }
