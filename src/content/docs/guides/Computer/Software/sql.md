---
title: SQL
---


SQL provides several commands to manipulate data:

- **`SELECT`**: Retrieve data from the database.
- **`INSERT`**: Add new rows to a table.
- **`UPDATE`**: Modify the values of existing rows.
- **`DELETE`**: Remove rows from a table.

## Aggregate Functions

SQL includes aggregate functions that operate on all the values in a column:

- **`COUNT`**: Counts the number of values.
- **`SUM`**: Calculates the total sum of values.
- **`AVG`**: Computes the average of the values.
- **`MAX`**: Finds the highest value.
- **`MIN`**: Finds the lowest value.

You can use the `DISTINCT` keyword before the argument to exclude duplicate values before applying the function.

## Special Case: `COUNT(*)`

- `COUNT(*)` counts **all rows** in a table, including those with `NULL` values.
- Other forms of `COUNT` (e.g. `COUNT(column_name)`) ignore `NULL` values.
