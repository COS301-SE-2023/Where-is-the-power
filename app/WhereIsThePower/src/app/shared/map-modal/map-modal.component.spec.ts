import { TestBed, ComponentFixture } from '@angular/core/testing';
import { MapModalComponent } from "./map-modal.component";
import { CUSTOM_ELEMENTS_SCHEMA,ChangeDetectorRef} from '@angular/core';
import { MapSuburbsService } from './map-suburbs.service';
import { HttpClientModule } from '@angular/common/http'; // Import HttpClientModule
import { IonContent, ModalController, AngularDelegate} from '@ionic/angular';


import { UserLocationService } from '../../user-location.service';



// import { environment } from 'src/environments/environment';
// declare let mapboxgl: any;

describe("MapModalComponent", () =>{
    let component: MapModalComponent;
    let fixture: ComponentFixture<MapModalComponent>;

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
    });

    it('should create the MapModalComponent', () => {
        expect(component).toBeTruthy();
    });

    it("should render the map", () =>{

    });
}
);