// js/register.js
document.addEventListener("DOMContentLoaded", () => {
    const registerForm = document.getElementById("registerForm");
  
    registerForm.addEventListener("submit", async (event) => {
      event.preventDefault();
  
      const username = document.getElementById("username").value;
      const password = document.getElementById("password").value;
  
      try {
        const response = await fetch("http://127.0.0.1:3000/register", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ username, password }),
        });
  
        if (!response.ok) {
          throw new Error("Erro ao registrar. Será que o username já existe?");
        }
  
        const data = await response.json();
        alert(data); // "User created with ID = X"
  
        // Redireciona pra tela de login
        window.location.href = "login.html";
      } catch (error) {
        alert("Erro ao registrar: " + error.message);
      }
    });
  });
  