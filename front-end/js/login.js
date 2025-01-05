// js/login.js
document.addEventListener("DOMContentLoaded", () => {
    const loginForm = document.getElementById("loginForm");
  
    loginForm.addEventListener("submit", async (event) => {
      event.preventDefault();
  
      const username = document.getElementById("username").value;
      const password = document.getElementById("password").value;
  
      try {
        // Mude a URL conforme onde seu backend Axum estiver rodando:
        const response = await fetch("http://localhost:3000/login", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ username, password }),
        });
  
        if (!response.ok) {
          throw new Error("Credenciais inválidas ou erro na requisição");
        }
  
        // Supondo que sua API retorna o token como string pura
        const token = await response.text();
        localStorage.setItem("jwt_token", token);
  
        alert("Login realizado com sucesso!");
        // Redireciona para página de eventos
        window.location.href = "events.html";
      } catch (error) {
        alert("Erro ao fazer login: " + error.message);
      }
    });
  });
  