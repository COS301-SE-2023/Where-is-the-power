/// <reference types="cypress" /> 

describe('My First Test', () => {
  it('Visits the initial project page', () => {
    cy.visit('/tabs/tab-schedule')
    cy.contains('Schedule')
    cy.contains('Saved')
    cy.contains('Navigate')
    cy.contains('Statistics')
    cy.contains('Area')
  })
})


describe('Navigation Test', () => {
  it('should navigate to different tabs', () => {
    cy.visit('/')
    // make sure we are on navigate page initially
    cy.location('pathname').should('eq', '/tabs/tab-navigate')
    
    cy.get('[data-cy="tab-saved"]').click()
    cy.location('pathname').should('eq', '/tabs/tab-saved')

    cy.get('[data-cy="tab-schedule"]').click()
    cy.location('pathname').should('eq', '/tabs/tab-schedule')

    cy.get('[data-cy="tab-statistics"]').click()
    cy.location('pathname').should('eq', '/tabs/tab-statistics')

    cy.get('[data-cy="tab-navigate"]').click()
    cy.location('pathname').should('eq', '/tabs/tab-navigate')

  })
})

describe('Canvas Map Test', () => {
  it('should have a canvas map', () => {
    cy.visit('/tabs/tab-navigate')
    cy.get('.mapboxgl-canvas').should('exist')
  })
 })


 describe('Authentication Test', () => {
  it('should create a new user', () => {
    cy.visit('/tabs/tab-profile')
    cy.get('[data-cy="sign-up-button"]').click()
    cy.get('[data-cy="input-fn"]').type("Bob")
    cy.get('[data-cy="input-ln"]').type("Marley")
    cy.get('[data-cy="input-email"]').type("bobmarley@gmail.com")
    cy.get('[data-cy="input-password"]').type("@bobmarley1")

    cy.get('[data-cy="btn-confirm-signup"]').click()

    cy.get('[data-cy="Welcome-text"]').should("to.have.text"," Welcome Bob !")
  })
  
  it('should log in as Bob Marley', () => {
    cy.visit('/tabs/tab-profile')
    cy.get('[data-cy="login-button"]').click()
    cy.get('[data-cy="login-email-input"]').type("bobmarley@gmail.com")
    cy.get('[data-cy="login-password-input"]').type("@bobmarley1")

    cy.get('[data-cy="btn-login-confirm"]').click()
    cy.get('[data-cy="Welcome-text"]').should("to.have.text"," Welcome Bob !")
  })
  
  it('should log out', () => {
    cy.visit('/tabs/tab-profile')
    cy.get('[data-cy="login-button"]').click()
    cy.get('[data-cy="login-email-input"]').type("bobmarley@gmail.com")
    cy.get('[data-cy="login-password-input"]').type("@bobmarley1")

    cy.get('[data-cy="btn-login-confirm"]').click()
    cy.get('[data-cy="logout-button"]').click()
    cy.get('[data-cy="Welcome-text"]').should("not.exist")
  })
  
  
 })
