const themeInput = document.getElementById("theme-input");
const form = document.getElementById("theme-form");

const changeTheme = async (e) => {
  const theme = e.target.value;

  await fetch(form.action, {
    method: "post",
    headers: {
      "X-Requested-With": "XMLHttpRequest",
    },
    body: new URLSearchParams({ theme }),
  });

  document.documentElement.className = theme;
  themeInput.value = theme === "dark" ? "light" : "dark";
};

themeInput.addEventListener("change", changeTheme);
