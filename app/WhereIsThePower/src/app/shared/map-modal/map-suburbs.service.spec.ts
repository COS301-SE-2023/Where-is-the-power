import { TestBed } from '@angular/core/testing';

import { MapSuburbsService } from './map-suburbs.service';

describe('MapSuburbsService', () => {
  let service: MapSuburbsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MapSuburbsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
