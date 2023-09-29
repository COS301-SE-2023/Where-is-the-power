/// <reference types="cypress" /> 

  
  describe('Navigation Test', () => {
    
    it('should navigate to different tabs', () => {
      cy.viewport('iphone-6')
      cy.visit('/tabs/tab-saved')

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

    it('should ask for location access', () => {
      // cy.window().then((win) => {
      //   cy.stub(win.navigator.geolocation, 'getCurrentPosition').throws(new Error('Geolocation is disabled.'));
      // });
      cy.viewport('iphone-6')
      cy.visit('/tabs/tab-navigate')
    });
    
    it('should have a canvas map', () => {
        cy.viewport('iphone-6')
        cy.visit('/tabs/tab-navigate')
        // Stub the navigator.geolocation API

        cy.get('.alert-button-inner').click()
        cy.get('[data-cy="turn-on-location-button"]').click()
        
        // cy.get('.mapboxgl-canvas').should('exist')
    })
   })
  
  
  //  describe('Authentication Test', () => {
  
  //   it('should create a new user', () => {
  //     cy.viewport('iphone-6')
  //     cy.visit('/tabs/tab-profile')
  //     cy.get('.in-toolbar > .list-md > :nth-child(2)').click()
  //     cy.get('[data-cy="input-fn"]').type("Bob")
  //     cy.get('[data-cy="input-ln"]').type("Marley")
  //     cy.get('[data-cy="input-email"]').type("bobmarley@gmail.com")
  //     cy.get('[data-cy="input-password"]').type("@bobmarley1")
  
  //     cy.get('[data-cy="btn-confirm-signup"]').click()
  
  //     // cy.get('[data-cy="Welcome-text"]').should("to.have.text"," Welcome Bob !")
  //   })
    
  //   it('should log in as Bob Marley', () => {
  //     cy.viewport('iphone-6')
  //     cy.visit('/tabs/tab-profile')
  //     cy.get('.in-toolbar > .list-md > :nth-child(1)').click()
  //     cy.get('[data-cy="login-email-input"]').type("bobmarley@gmail.com")
  //     cy.get('[data-cy="login-password-input"]').type("@bobmarley1")
  
  //     // cy.get('[data-cy="btn-login-confirm"]').click()
  //     // cy.get('[data-cy="Welcome-text"]').should("to.have.text"," Welcome Bob !")
  //   })
    
  //   it('should log out', () => {
  //     cy.viewport('iphone-6')
  //     cy.visit('/tabs/tab-profile')
  //     cy.get('.in-toolbar > .list-md > :nth-child(2)').click()
  //     cy.get('[data-cy="input-fn"]').type("Bob")
  //     cy.get('[data-cy="input-ln"]').type("Marley")
  //     cy.get('[data-cy="input-email"]').type("bobmarley@gmail.com")
  //     cy.get('[data-cy="input-password"]').type("@bobmarley1")
  
  //     cy.get('[data-cy="btn-confirm-signup"]').click()
  //     cy.get('[data-cy="logout-button"]').click()
  //     cy.get('[data-cy="login-button"]').should('exist');
  //   })
    
    
  //  })
  