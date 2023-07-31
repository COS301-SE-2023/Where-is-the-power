import { TestBed, ComponentFixture, waitForAsync } from '@angular/core/testing';
import { MapModalComponent } from "./map-modal.component";
import { CUSTOM_ELEMENTS_SCHEMA,ChangeDetectorRef} from '@angular/core';
import { MapSuburbsService } from './map-suburbs.service';
import { HttpClientModule } from '@angular/common/http'; // Import HttpClientModule
import { IonContent, ModalController, AngularDelegate} from '@ionic/angular';
import { of } from 'rxjs';


import { UserLocationService } from '../../user-location.service';

declare let mapboxgl: any;



// import { environment } from 'src/environments/environment';
// declare let mapboxgl: any;

describe("MapModalComponent", () =>{
    let component: MapModalComponent;
    let fixture: ComponentFixture<MapModalComponent>;
    let mapSuburbsService: MapSuburbsService;
    let userLocationService: UserLocationService;

    beforeEach(() => {
        // Initialize the testing environment and create the component fixture
        TestBed.configureTestingModule({
            declarations: [MapModalComponent],
            schemas: [CUSTOM_ELEMENTS_SCHEMA],
            providers: [MapSuburbsService, ModalController, UserLocationService,ChangeDetectorRef, AngularDelegate], // Provide the MapSuburbsService if needed
            imports: [HttpClientModule], // Import HttpClientModule
        }).compileComponents();
        fixture = TestBed.createComponent(MapModalComponent);
        component = fixture.componentInstance;
        mapSuburbsService = TestBed.inject(MapSuburbsService);
        userLocationService = TestBed.inject(UserLocationService);
        fixture.detectChanges();
    });

    it('should create the MapModalComponent', () => {
        expect(component).toBeTruthy();
    });

    it("should render the map", () =>{

    });
    xit('should call getSuburbData() and populate map with polygons', (done) => {
        spyOn(mapSuburbsService, 'getSuburbData').and.callThrough();
        spyOn(component, 'populatePolygons');
        // spyOn(component, 'ngAfterViewInit').and.callThrough;
    
        // Trigger the ngAfterViewInit function
        try {
            component.ngAfterViewInit();
        } catch(error) {
            fail('ngAfterViewInit threw an error: ' + error);
        }

        

        
    
        // Check if getSuburbData was called
        expect(mapSuburbsService.getSuburbData).toHaveBeenCalled();
    
        fixture.whenStable().then(() => {
            // Check if populatePolygons was called after the subscription completes
            expect(component.populatePolygons).toHaveBeenCalled();
            done();
        })
        
        // expect(component.map).toBeTruthy();
    
        // Additional tests can be done based on the behavior of your component.
        // For instance, you can check if the map is initialized with the correct properties.
      });
    
}
);