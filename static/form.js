const typeField = document.getElementById("type");

function change(type) {
    typeField.value = type;

    document.querySelectorAll(".form-extras").forEach(element => {
      if (element.id !== type) {
        element.style.display = "none";
      }
    });

    document.getElementById(type).style.display = "block";
}