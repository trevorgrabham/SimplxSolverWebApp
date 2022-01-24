SimplexSolverWebApp
===================

### Introduction

This document details the plan for the development of the SimplexSolverWebApp.

It is intended for current and future developers, designers, and testers working on the SimplexSolverWebApp. This plan will include a summary of:

- how the system will function
- the scope of the project
- the technologies used for development
- the metrics used for testing and evaluation

### Overall Description

Students learning to solve linear programs by hand can often come to incorrect solutions due to the large number of calculations involved and how incorrect calculations propagate through the calculations. Common aids in these calculations can produce correct solutions, but do not show the intermediate steps taken to get there. This can allow students to see that their calculations have gone wrong, but does not help them conclude where the mistakes have occurred. The SimplexSolverWebApp will solve this by displaying the intermediate steps that are gone through when using simplex tableaus to solve linear programs by hand. 

The current applications that allow users to see the intermediate tables generated on the way to a solution do not allow the user to specify the selection criteria that are used for entering and exiting variables or the algorithm used to solve the tableau, and do not support artificial variables. The SimplexSolverWebApp will allow the user to specify these so that the intermediate steps shown to the solution match the steps that the user expects to see. 

### Users 

The users will be students in the process of learning to solve linear programs by hand. 

### Functionality

- Users should be able to select the dimensions of their initial tableau, the algorithm used to solve it, and the criteria used to select entering and exiting variables. They should then be able to fill out their starting tableau and be presented with each intermediate tableau on the way to a solution.
- Users should be able to navigate to provided resources on how to use the application and some background information on what their different options are for the algorithm and variable selection methods.

### Platform

The application will be developed using Next.js and React components to allow for faster responses with server-side rendering. 

The front-end application will connect to a REST API build with Rust and Rocket. 

At the current time, there are no plans to implement any data storage on the server-side, so there will not be any database connections currently needed. With no user information being stored on the back-end, there will be no need for user authentication as well. 

### Development Responsibilities

The developer of the SimplexSolverWebApp will be responsible for the development of both the front and back-end applications as well as any maintenance and release management.

### User Class and Characteristics

All users will be considered equal, as the SimplexSolverWebApp at this point does not have a need for any special classes of users.

### System Features 

#### Functional Requirements

- Users should be able to specify the dimensions of their initial tableau and have an empty tableau of that size be presented to them to update with their tableau values.
- Users should be able to select different parameters to run their calculations with, such as the algorithm choice and the variable selection criteria.
- Users should be able to see all of the tableau transformations that have occured between the initial tableau and the solved tableau.

#### User Interfaces

- Front-end software: Next.js and React
- Back-end software: Rust and Rocket
- Database software: N/A

#### Hardware Interfaces

- All operating systems through their default web browsers.
- Mobile web browsers.

#### Performance Requirements

- The solution should be presented to the user within 2 seconds for reasonable sized tableaus.
- At each step in the solution, the current tableau should be displayed for the user along with the entering and leaving variables.

#### Safety Requirements

Not applicable at the current time as no user information is being collected.

#### Security Requirements

Not applicable at the current time as no user information is being collected.

#### Software Quality Attributes

- Availability: Because the application is not critical for the end users, a goal of 99% availability should suffice.
- Correctness: The application should always produce a correct optimal solution for the user. If multiple optimal solutions exist, the application should come to the same solution that the user would come to by hand given the criteria selected by the user. If no optimal solution exists then the application should produce intermediate tableaus until it is able to determine that the linear program is unbounded.
- Maintainability: The application should be well documented and modular so that it is able to scale in the future if updates are to be added.
- Usability: The application should contain a place for users to review documentation on correct use and the meaning of all of the different selection criteria that can be used. The rest of the interface should be easy to use for the users and be self explanitory.

_______________________________________________________________________________________________

### Development Team

Trevor Grabham