package vsentinel

import (
	"time"
)

// DetectionSeverity represents the severity level of a detection.
type DetectionSeverity string

const (
	SeverityCritical      DetectionSeverity = "CRITICAL"
	SeverityHigh          DetectionSeverity = "HIGH"
	SeverityMedium        DetectionSeverity = "MEDIUM"
	SeverityLow           DetectionSeverity = "LOW"
	SeverityInformational DetectionSeverity = "INFORMATIONAL"
)

// DetectionStatus represents the status of a detection.
type DetectionStatus string

const (
	DetectionStatusNew         DetectionStatus = "NEW"
	DetectionStatusInProgress  DetectionStatus = "IN_PROGRESS"
	DetectionStatusResolved    DetectionStatus = "RESOLVED"
	DetectionStatusFalsePositive DetectionStatus = "FALSE_POSITIVE"
	DetectionStatusIgnored     DetectionStatus = "IGNORED"
)

// HostState represents the state of a host.
type HostState string

const (
	HostStateOnline  HostState = "ONLINE"
	HostStateOffline HostState = "OFFLINE"
	HostStateIsolated HostState = "ISOLATED"
	HostStateUnknown HostState = "UNKNOWN"
)

// HostPlatform represents the platform of a host.
type HostPlatform string

const (
	PlatformWindows HostPlatform = "WINDOWS"
	PlatformLinux   HostPlatform = "LINUX"
	PlatformMacOS   HostPlatform = "MACOS"
	PlatformUnknown HostPlatform = "UNKNOWN"
)

// IncidentSeverity represents the severity of an incident.
type IncidentSeverity string

const (
	IncidentSeverityCritical IncidentSeverity = "CRITICAL"
	IncidentSeverityHigh     IncidentSeverity = "HIGH"
	IncidentSeverityMedium   IncidentSeverity = "MEDIUM"
	IncidentSeverityLow      IncidentSeverity = "LOW"
)

// IncidentStatus represents the status of an incident.
type IncidentStatus string

const (
	IncidentStatusNew          IncidentStatus = "NEW"
	IncidentStatusInProgress   IncidentStatus = "IN_PROGRESS"
	IncidentStatusOnHold       IncidentStatus = "ON_HOLD"
	IncidentStatusResolved     IncidentStatus = "RESOLVED"
	IncidentStatusClosed       IncidentStatus = "CLOSED"
	IncidentStatusFalsePositive IncidentStatus = "FALSE_POSITIVE"
)

// IncidentPhase represents the phase of an incident.
type IncidentPhase string

const (
	PhaseIdentification  IncidentPhase = "IDENTIFICATION"
	PhaseContainment     IncidentPhase = "CONTAINMENT"
	PhaseEradication     IncidentPhase = "ERADICATION"
	PhaseRecovery        IncidentPhase = "RECOVERY"
	PhaseLessonsLearned  IncidentPhase = "LESSONS_LEARNED"
)

// IOCType represents the type of an IOC.
type IOCType string

const (
	IOCTypeIP         IOCType = "IP"
	IOCTypeDomain     IOCType = "DOMAIN"
	IOCTypeURL        IOCType = "URL"
	IOCTypeHashMD5    IOCType = "HASH_MD5"
	IOCTypeHashSHA1   IOCType = "HASH_SHA1"
	IOCTypeHashSHA256 IOCType = "HASH_SHA256"
	IOCTypeEmail      IOCType = "EMAIL"
	IOCTypeCertificate IOCType = "CERTIFICATE"
)

// IOCConfidence represents the confidence level of an IOC.
type IOCConfidence string

const (
	ConfidenceLow    IOCConfidence = "LOW"
	ConfidenceMedium IOCConfidence = "MEDIUM"
	ConfidenceHigh   IOCConfidence = "HIGH"
)

// Detection represents a security detection.
type Detection struct {
	ID          string           `json:"id"`
	Title       string           `json:"title"`
	Description string           `json:"description,omitempty"`
	Severity    DetectionSeverity `json:"severity"`
	Status      DetectionStatus  `json:"status"`
	Techniques  []string         `json:"techniques,omitempty"`
	Tactics     []string         `json:"tactics,omitempty"`
	Host        *Host            `json:"host,omitempty"`
	Indicators  []Indicator      `json:"indicators,omitempty"`
	RawData     map[string]any   `json:"raw_data,omitempty"`
	Assignee    string           `json:"assignee,omitempty"`
	Notes       []DetectionNote  `json:"notes,omitempty"`
	Tags        []string         `json:"tags,omitempty"`
	CreatedAt   time.Time        `json:"created_at"`
	UpdatedAt   time.Time        `json:"updated_at"`
}

// Indicator represents an indicator within a detection.
type Indicator struct {
	Type        string `json:"type"`
	Value       string `json:"value"`
	Description string `json:"description,omitempty"`
}

// DetectionNote represents a note on a detection.
type DetectionNote struct {
	ID        string    `json:"id"`
	Content   string    `json:"content"`
	Author    string    `json:"author"`
	CreatedAt time.Time `json:"created_at"`
}

// DetectionList represents a list of detections.
type DetectionList struct {
	Items   []Detection `json:"items"`
	Total   int         `json:"total"`
	Limit   int         `json:"limit"`
	Offset  int         `json:"offset"`
	HasMore bool        `json:"has_more"`
}

// Host represents a managed host/endpoint.
type Host struct {
	ID            string      `json:"id"`
	Hostname      string      `json:"hostname"`
	Platform      HostPlatform `json:"platform"`
	State         HostState   `json:"state"`
	IPAddresses   []string    `json:"ip_addresses,omitempty"`
	OSVersion     string      `json:"os_version,omitempty"`
	AgentVersion  string      `json:"agent_version,omitempty"`
	IsIsolated    bool        `json:"is_isolated"`
	LastSeenAt    *time.Time  `json:"last_seen_at,omitempty"`
	FirstSeenAt   *time.Time  `json:"first_seen_at,omitempty"`
	Tags          []string    `json:"tags,omitempty"`
	CreatedAt     time.Time   `json:"created_at"`
	UpdatedAt     time.Time   `json:"updated_at"`
}

// HostList represents a list of hosts.
type HostList struct {
	Items   []Host `json:"items"`
	Total   int    `json:"total"`
	Limit   int    `json:"limit"`
	Offset  int    `json:"offset"`
	HasMore bool   `json:"has_more"`
}

// Incident represents a security incident.
type Incident struct {
	ID          string            `json:"id"`
	Title       string            `json:"title"`
	Description string            `json:"description,omitempty"`
	Severity    IncidentSeverity  `json:"severity"`
	Status      IncidentStatus    `json:"status"`
	Phase       IncidentPhase     `json:"phase,omitempty"`
	Hosts       []string          `json:"hosts,omitempty"`
	Detections  []string          `json:"detections,omitempty"`
	Assignee    string            `json:"assignee,omitempty"`
	Timeline    []TimelineEntry   `json:"timeline,omitempty"`
	Notes       []IncidentNote    `json:"notes,omitempty"`
	Tags        []string          `json:"tags,omitempty"`
	CreatedAt   time.Time         `json:"created_at"`
	UpdatedAt   time.Time         `json:"updated_at"`
}

// TimelineEntry represents an entry in the incident timeline.
type TimelineEntry struct {
	Timestamp   time.Time `json:"timestamp"`
	Action      string    `json:"action"`
	Description string    `json:"description,omitempty"`
	User        string    `json:"user,omitempty"`
}

// IncidentNote represents a note on an incident.
type IncidentNote struct {
	ID         string    `json:"id"`
	Content    string    `json:"content"`
	Author     string    `json:"author"`
	Visibility string    `json:"visibility,omitempty"`
	CreatedAt  time.Time `json:"created_at"`
}

// IncidentList represents a list of incidents.
type IncidentList struct {
	Items   []Incident `json:"items"`
	Total   int        `json:"total"`
	Limit   int        `json:"limit"`
	Offset  int        `json:"offset"`
	HasMore bool       `json:"has_more"`
}

// IOC represents an indicator of compromise.
type IOC struct {
	ID            string         `json:"id"`
	Type          IOCType        `json:"type"`
	Value         string         `json:"value"`
	Confidence    IOCConfidence  `json:"confidence"`
	ThreatActor   string         `json:"threat_actor,omitempty"`
	Campaign      string         `json:"campaign,omitempty"`
	MalwareFamily string         `json:"malware_family,omitempty"`
	Tags          []string       `json:"tags,omitempty"`
	Description   string         `json:"description,omitempty"`
	References    []string       `json:"references,omitempty"`
	FirstSeen     *time.Time     `json:"first_seen,omitempty"`
	LastSeen      *time.Time     `json:"last_seen,omitempty"`
	ExpiresAt     *time.Time     `json:"expires_at,omitempty"`
	CreatedAt     time.Time      `json:"created_at"`
	UpdatedAt     time.Time      `json:"updated_at"`
}

// IOCList represents a list of IOCs.
type IOCList struct {
	Items   []IOC `json:"items"`
	Total   int   `json:"total"`
	Limit   int   `json:"limit"`
	Offset  int   `json:"offset"`
	HasMore bool  `json:"has_more"`
}

// IOCCheckResult represents the result of checking an IOC.
type IOCCheckResult struct {
	IOC           *IOC   `json:"ioc,omitempty"`
	IsMalicious   bool   `json:"is_malicious"`
	Confidence    IOCConfidence `json:"confidence"`
	ThreatActor   string `json:"threat_actor,omitempty"`
	MalwareFamily string `json:"malware_family,omitempty"`
	RelatedIOCs   []IOC  `json:"related_iocs,omitempty"`
}

// ThreatActor represents a threat actor/group.
type ThreatActor struct {
	Name         string    `json:"name"`
	Aliases      []string  `json:"aliases,omitempty"`
	Country      string    `json:"country,omitempty"`
	Motivation   string    `json:"motivation,omitempty"`
	MITREID      string    `json:"mitre_id,omitempty"`
	FirstSeen    *time.Time `json:"first_seen,omitempty"`
	LastSeen     *time.Time `json:"last_seen,omitempty"`
	Tags         []string  `json:"tags,omitempty"`
	Description  string    `json:"description,omitempty"`
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`
}

// ThreatActorList represents a list of threat actors.
type ThreatActorList struct {
	Items   []ThreatActor `json:"items"`
	Total   int           `json:"total"`
	Limit   int           `json:"limit"`
	Offset  int           `json:"offset"`
	HasMore bool          `json:"has_more"`
}

// Campaign represents a threat campaign.
type Campaign struct {
	ID           string     `json:"id"`
	Name         string     `json:"name"`
	ThreatActor  string     `json:"threat_actor,omitempty"`
	Description  string     `json:"description,omitempty"`
	Status       string     `json:"status,omitempty"`
	StartDate    *time.Time `json:"start_date,omitempty"`
	EndDate      *time.Time `json:"end_date,omitempty"`
	Tags         []string   `json:"tags,omitempty"`
	CreatedAt    time.Time  `json:"created_at"`
	UpdatedAt    time.Time  `json:"updated_at"`
}

// CampaignList represents a list of campaigns.
type CampaignList struct {
	Items   []Campaign `json:"items"`
	Total   int        `json:"total"`
	Limit   int        `json:"limit"`
	Offset  int        `json:"offset"`
	HasMore bool       `json:"has_more"`
}