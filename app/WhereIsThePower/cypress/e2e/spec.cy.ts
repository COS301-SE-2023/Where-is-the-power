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
