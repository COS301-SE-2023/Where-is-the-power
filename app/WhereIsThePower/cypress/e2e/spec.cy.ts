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
