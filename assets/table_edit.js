"use strict";

//console.log('WORKER: executing.');

/* A version number is useful when updating the worker logic,
   allowing you to remove outdated cache entries during the update.
*/
var version = 'v1.0.0::';

console.log("hi from js");

document.addEventListener('DOMContentLoaded', function () {
    const squeezeButton = document.getElementById('squeezeTodos');
    squeezeButton.addEventListener('click', squeezeTodos);
  
    function squeezeTodos() {
      const calendar = document.getElementById('calendar');
      const todos = document.querySelectorAll('.todo');
      const cells = calendar.querySelectorAll('td');
      const todosPerCell = Math.ceil(todos.length / cells.length);
      console.log("hi from js");

      // Clear existing todos in the calendar cells
      cells.forEach(cell => cell.innerHTML = '');
  
      // Distribute todos equally
      let cellIndex = 0;
      todos.forEach((todo, index) => {
        cells[cellIndex].appendChild(todo);
        if ((index + 1) % todosPerCell === 0) {
          cellIndex++;
        }
      });
    }
  
    // Enable drag and drop functionality
    const todos = document.querySelectorAll('.todo');
    todos.forEach(todo => {
      todo.setAttribute('draggable', true);
      todo.addEventListener('dragstart', dragStart);
    });
  
    const cells = document.querySelectorAll('td');
    cells.forEach(cell => {
      cell.addEventListener('dragover', dragOver);
      cell.addEventListener('drop', drop);
    });
  
    function dragStart(event) {
      event.dataTransfer.setData('text/plain', event.target.id);
      event.dataTransfer.dropEffect = 'move';
    }
  
    function dragOver(event) {
      event.preventDefault();
      event.dataTransfer.dropEffect = 'move';
    }
  
    function drop(event) {
      event.preventDefault();
      const id = event.dataTransfer.getData('text/plain');
      const todo = document.getElementById(id);
      event.target.appendChild(todo);
    }
  });
  