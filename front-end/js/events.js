// js/events.js
document.addEventListener("DOMContentLoaded", () => {
    const token = localStorage.getItem("jwt_token");
    if (!token) {
      alert("Você precisa estar logado para acessar esta página!");
      window.location.href = "login.html";
      return;
    }
  
    // Botão de logout
    const btnLogout = document.getElementById("btnLogout");
    btnLogout.addEventListener("click", () => {
      localStorage.removeItem("jwt_token");
      window.location.href = "login.html";
    });
  
    // FORM: Criar evento
    createEventForm.addEventListener("submit", async (event) => {
      event.preventDefault();
    
      const name = document.getElementById("eventName").value;
      const description = document.getElementById("eventDescription").value;
      const eventType = document.getElementById("eventType").value;
      const date = document.getElementById("eventDate").value;
      const user_id = 0;
    
      try {
        const response = await fetch("http://localhost:3000/event", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            event: {
              id: 0,
              name,
              description,
              event_type: eventType,
              date,
              user_id
            },
            token: localStorage.getItem("jwt_token"),
          }),
        });
    
        if (!response.ok) {
          throw new Error("Falha ao criar evento");
        }
    
        const data = await response.json();
        alert(data); // "Event created with ID = X"
      } catch (err) {
        alert("Erro ao criar evento: " + err.message);
      }
    });    
  
    // FORM: Consultar evento
    const consultEventForm = document.getElementById("consultEventForm");
    consultEventForm.addEventListener("submit", async (event) => {
      event.preventDefault();
  
      const eventName = document.getElementById("consultName").value;
      const userId = document.getElementById("consultUserId").value;
  
      try {
        const response = await fetch("http://localhost:3000/event", {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
          
        });
  
        if (!response.ok) {
          throw new Error("Erro ao buscar evento");
        }
  
        alert("Este GET não tá configurado para receber body no Axum, revise seu endpoint!");
        
        // Se tudo desse certo, você receberia uma lista de eventos
        const eventsData = []; // = await response.json();
  
        // Renderiza a lista
        renderEvents(eventsData);
      } catch (err) {
        alert("Erro: " + err.message);
      }
    });
  
    // FORM: Deletar evento
    const deleteEventForm = document.getElementById("deleteEventForm");
    deleteEventForm.addEventListener("submit", async (event) => {
      event.preventDefault();
  
      const userId = document.getElementById("deleteUserId").value;
      const eventId = document.getElementById("deleteEventId").value;
  
      try {
        const response = await fetch("http://localhost:3000/event", {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            user_id: parseInt(userId),
            event_id: eventId,
            token
          }),
        });
  
        if (!response.ok) {
          throw new Error("Erro ao deletar evento");
        }
  
        const data = await response.json();
        alert(data); // "Event deleted"
      } catch (err) {
        alert("Erro ao deletar evento: " + err.message);
      }
    });
  });
  
  function renderEvents(events) {
    const eventsListDiv = document.getElementById("eventsList");
    eventsListDiv.innerHTML = "";
  
    if (!events || events.length === 0) {
      eventsListDiv.innerHTML = "<p>Nenhum evento encontrado.</p>";
      return;
    }
  
    // Cria uma tabela ou lista
    const ul = document.createElement("ul");
    events.forEach((evt) => {
      const li = document.createElement("li");
      li.textContent = `ID: ${evt.id} - ${evt.name} (${evt.event_type}), Desc: ${evt.description}, Data: ${evt.date}`;
      ul.appendChild(li);
    });
  
    eventsListDiv.appendChild(ul);
  }
  