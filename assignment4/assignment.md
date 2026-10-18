# Assignment 4

## 1 Overview

You will implement, without logical errors, various control structures such as conditionals, loops, and
branching, in multiple programming languages. This assignment will demonstrate your mastery of syntax
and control flow in different paradigms.

## 2 Application Prompt

You are tasked with developing a small application for managing employee schedules at a company. The
company operates 7 days a week and allows employees to choose shifts in the morning, afternoon, or
evening. The application should manage and output the following:

1. **Input and Storage**
   - The application should collect employee names and their preferred shifts (morning, afternoon, or
     evening) for each day of the week. Store this information in an appropriate data structure.
2. **Scheduling Logic**
   - Assign shifts for employees. Ensure that:
     - No employee works more than one shift per day.
     - An employee can work a maximum of 5 days per week.
     - The company must have at least 2 employees per shift per day. If fewer than 2 employees are
       available for a shift, randomly assign additional employees who have not worked 5 days yet.
3. **Shift Conflicts**
   - Implement logic to detect and resolve conflicts where an employee's preferred shift is full for a
     given day. If a conflict occurs, assign the employee to another available shift on the same or
     next day.
4. **Output**
   - After scheduling the shifts, the application should output the final schedule for the week in a
     readable format, indicating which employee is assigned to each shift on each day.
5. **Bonus (optional)**
   - Allow employees to indicate shift preferences with a priority ranking (e.g., morning is their
     first preference, evening is their second). Implement logic to accommodate these preferences as
     much as possible, while still ensuring company shift requirements are met.
   - In addition to the core requirements of implementing control structures in multiple languages,
     students may choose to implement a Graphical User Interface (GUI) for both the input (employee
     names and shift preferences) and the output (final employee schedule) in one of the languages.

## 3 Programming Language Suggestions

You are free to choose any two languages, but the languages must be different enough that there's a
distinct contrast. For example, do not pick C++ and C#. Here's a list of suggested languages:

- Python
- Java
- C++
- JavaScript
- Go
- Rust

## 4 Deliverable

You will submit the source code for the two programming languages that you chose. This can be done by
submitting the link to your public GitHub repo for this assignment. Please include a screenshot of your
final schedule output. You can submit the screenshot(s) and the GitHub link in one document (.docx and
.pdf) to upload.
